use gl;
use sdl2;

pub struct Program {
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    pub context: sdl2::video::GLContext,
    pub event_pump: sdl2::EventPump,
}

impl Program {
    pub fn new(title: String, width: u32, height: u32) -> Result<Self, String> {
        let sdl = try!(sdl2::init());

        let event_pump = try!(sdl.event_pump());

        println!("init video");
        let video = try!(sdl.video());
        println!("init video done");
        // force vsync
        video.gl_set_swap_interval(1);

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
        gl_attr.set_context_flags().debug().set();

        // Load the system's OpenGL library
        try!(video.gl_load_library_default());
        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

        // Create our window (and make it usable with OpenGL)
        let window = try!(
            video
                .window(&title, width, height)
                .opengl()
                .build()
                .map_err(|_| "failed to initialise window")
        );

        let context = window.gl_create_context().unwrap();
        Ok(Program {
            sdl: sdl,
            window: window,
            context: context,
            event_pump: event_pump,
        })
    }
}
