use bevy::prelude::*;
use super::link::Link;
use crate::layer2::interface::Interface;

pub fn transmit_frames(links: Query<&Link>, mut interfaces: Query<&mut Interface>) {
    for link in links.iter() {
        // Transmit frame from link.0 to link.1
        Link::transmit_frame(link.0, link.1, &mut interfaces);

        // Transmit frame from link.1 to link.0
        Link::transmit_frame(link.1, link.0, &mut interfaces);
    }
}