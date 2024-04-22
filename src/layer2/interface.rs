use super::{
    address::MacAddress,
    arp::{ArpOperation, ArpTable},
    pdu::{EthernetFrame, EthernetPayload},
};
use crate::layer3::address::{IpAddr, Ipv4Addr, Ipv6Addr};
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct SourceInterface;
#[derive(Component)]
pub struct DestinationInterface;

pub struct Queue<T> {
    elements: VecDeque<T>,
    capacity: u32,
}

impl<T> Queue<T> {
    // Creates a new empty queue
    pub fn new(capacity: u32) -> Self {
        Queue {
            elements: VecDeque::new(),
            capacity,
        }
    }

    // Adds an item to the back of the queue
    pub fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    // Removes an item from the front of the queue
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    // Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Returns the number of items in the queue
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    // Peeks at the first item in the queue without removing it
    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }
}

#[derive(Debug)]
pub enum InterfaceType {
    FastEthernet,
    GigabitEthernet,
    TenGigabitEthernet,
}

#[derive(Component)]
pub enum Interface {
    Ethernet(EthernetInterface),
    Serial(SerialInterface),
}

impl Interface {
    pub fn attach_to_device(&mut self, device: Entity) {
        match self {
            Interface::Ethernet(interface) => {
                interface.device = Some(device);
            }
            Interface::Serial(_) => {}
        }
    }
}

#[derive(Component)]
pub struct EthernetInterface {
    pub interface_type: InterfaceType,
    pub device: Option<Entity>,
    pub mac_address: MacAddress,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_addresses: Vec<Ipv6Addr>,
    pub arp_table: ArpTable,
    pub in_queue: Queue<EthernetFrame>,
    pub out_queue: Queue<EthernetFrame>,
}

pub enum Direction {
    In,
    Out,
}

impl EthernetInterface {
    pub fn new(interface_type: InterfaceType) -> Self {
        Self {
            interface_type: interface_type,
            device: None,
            mac_address: MacAddress::random(),
            ipv4_address: None,
            ipv6_addresses: Vec::new(),
            arp_table: ArpTable::new(),
            in_queue: Queue::new(0x2000000),  // 32 MB
            out_queue: Queue::new(0x2000000), // 32 MB
        }
    }

    pub fn set_ipv4_address(&mut self, ipv4_address: Ipv4Addr) {
        self.ipv4_address = Some(ipv4_address);
    }

    pub fn add_ipv6_address(&mut self, ipv6_address: Ipv6Addr) {
        self.ipv6_addresses.push(ipv6_address);
    }

    pub fn enqueue_frame(&mut self, frame: EthernetFrame, direction: Direction) {
        match direction {
            Direction::In => self.in_queue.enqueue(frame),
            Direction::Out => self.out_queue.enqueue(frame),
        }
    }

    pub fn dequeue_frame(&mut self, direction: Direction) -> Option<EthernetFrame> {
        match direction {
            Direction::In => self.in_queue.dequeue(),
            Direction::Out => self.out_queue.dequeue(),
        }
    }

    pub fn process_frame(&mut self, frame: &EthernetFrame) {
        match &frame.payload {
            EthernetPayload::Dummy => {
                println!("Received dummy frame");
            }
            EthernetPayload::ARP(arp) => match arp.operation {
                ArpOperation::Request => {
                    println!("");
                    println!("Received ARP request");
                    let target_ip = &arp.target_ip;
                    println!("  Who has IP address {}?", target_ip);
                    let reply_frame = frame.arp_reply(arp, self.mac_address.clone());
                    self.enqueue_frame(reply_frame, Direction::Out);
                }
                ArpOperation::Reply => {
                    println!("");
                    println!("Received ARP reply");
                    let sender_ip = &arp.sender_ip;
                    let sender_mac = &arp.sender_mac;
                    println!("  {} is at {}", sender_ip, sender_mac);
                    self.arp_table
                        .add_entry(sender_ip.clone(), sender_mac.clone());
                }
            },
            EthernetPayload::ICMP => {
                println!("Received ICMP frame");
            }
            EthernetPayload::IPv4(ip_packet) => {
                println!("Received IP frame: {:?}", ip_packet);
            }
            _ => {
                println!("Received frame with unknown payload");
            }
        }
    }

    /// Short-circuits the queues by moving the first item from the in_queue to the out_queue
    /// This is useful for testing purposes
    pub fn short_circuit_queues(&mut self) {
        if let Some(item) = self.in_queue.dequeue() {
            self.out_queue.enqueue(item);
        }
    }
}

pub struct SerialInterface {
    pub ip_addresses: Vec<IpAddr>,
}

impl SerialInterface {
    pub fn new(ip_addresses: Vec<IpAddr>) -> Self {
        Self { ip_addresses }
    }
}
