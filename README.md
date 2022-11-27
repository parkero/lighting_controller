# Lighting Controller

This is a library that will generate smooth looping animations on a set of colored pixels based on user-selected input parameters. It is designed to be highly configurable and versatile, using custom color palettes called `rainbows` and multiple layered animation types that can be mixed and matched to allow for a diverse set of custom lighting effects. The designed use for this library is in calculating pleasing lighting effects to be used with addressable LED strips, though it may also be useful for any situation where smooth looping colored animations are desired. Since this project is intended for use on microcontrollers, it is coded without use of the standard library and uses only dependencies that are also `no_std` compatible.


I intend to provide a more thorough writeup and examples as part of this readme once the library is working as intended, so for now this readme will just give a general overview of the project and links to past working projects.

The work here is an attempt to consolidate and make more generic the LED lighting animations I have used on my previous projects in both rust and C/C++. The basis for this lighting controller began with my [IIDX deck](https://github.com/kiyoshigawa/IIDX_Deck) and some of the early animations can be seen in my [blog posts on the build](https://twa.ninja/blog/iidx_deck_-_build_log_-_part_4). The next project to expand on the concepts for this lighting controller was the [oMIDItone](https://github.com/kiyoshigawa/oMIDItone_Controller_V2). You can see the lighting in action on this [youtube video](https://www.youtube.com/watch?v=nIBvpmfh668) of the device. It is also being used to run [LED strip lighting in my office](https://github.com/kiyoshigawa/bl602-ws2811), though I am planning to use this library to update the lighting controller for my office once it is complete.


## License

Copyright (C) 2022 Tim Anderson

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software Foundation,
Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA
