# Utopia

A modular, unopinionated, lightweight, data-driven UI library focused on games and desktop applications. Written in Rust.

Greatly inspired by Druid and Iced.

## Features

* Modular design empowering everyone to create their own widgets, and match closely the target platform's capacities 
* Data-driven design
* Easy layout (including Flex layout and Stacks)
* Various goodies such as widget decorations and scrolling
* Basic widgets such as Image, Text and Labels
* Support for animations à la Flutter, safely animate *all* widgets through lenses. 
* Animation easing through [keyframes](https://docs.rs/keyframe/1.0.3/keyframe/)

## Roadmap 

* Handle futures, async components
* Proper caching of primitives / render optimisations / re-layout only when required 
* Extract Lenses to their own crate, and provide a proc macro
* General cleanup. Fix todos, write some documentation, ensure the basic design is correct 
* Write a `bevy` backend 
* Utility (form) widgets : 
	* Button
	* TextInput widget
	* Checkboxes,
	* Optionboxes,
	* OptionGroup
	* Slider,
	* Progress bar,
	* Select,
	* Switch,
* Validation process for Forms
* Lifecycle events ?
* Rich text
* Drag-and-drop
* Tabs widget
* Table widget
* Widget focus ? 
* Canvas ?
