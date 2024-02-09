pub const MAIN_CLASS: &'static str = "zombie/network/GameServer";

pub const CLASS_PATH_64: &'static str =
    "java/.;java/istack-commons-runtime.jar;java/jassimp.jar;java/javacord-2.0.17-shaded.jar;java/\
     javax.activation-api.jar;java/jaxb-api.jar;java/jaxb-runtime.jar;java/lwjgl.jar;java/\
     lwjgl-natives-linux.jar;java/lwjgl-glfw.jar;java/lwjgl-glfw-natives-linux.jar;java/\
     lwjgl-jemalloc.jar;java/lwjgl-jemalloc-natives-linux.jar;java/lwjgl-opengl.jar;java/\
     lwjgl-opengl-natives-linux.jar;java/lwjgl_util.jar;java/sqlite-jdbc-3.27.2.1.jar;java/\
     trove-3.0.3.jar;java/uncommons-maths-1.2.3.jar;java/commons-compress-1.18.jar";
pub const CLASS_PATH_32: &'static str =
    "java/.;commons-compress-1.18.jar;java/istack-commons-runtime.jar;java/jassimp.jar;java/\
     javacord-2.0.17-shaded.jar;java/javax.activation-api.jar;java/jaxb-api.jar;java/jaxb-runtime.\
     jar;java/lwjgl.jar;java/lwjgl-natives-windows-x86.jar;java/lwjgl-glfw.jar;java/\
     lwjgl-glfw-natives-windows-x86.jar;java/lwjgl-jemalloc.jar;java/\
     lwjgl-jemalloc-natives-windows-x86.jar;java/lwjgl-opengl.jar;java/\
     lwjgl-opengl-natives-windows-x86.jar;java/lwjgl_util.jar;java/sqlite-jdbc-3.27.2.1.jar;java/\
     trove-3.0.3.jar;java/uncommons-maths-1.2.3.jar";

pub const VM_ARGS_64: &'static str = "-Djava.awt.headless=true -Dzomboid.steam=1 \
                                      -Dzomboid.znetlog=1 -XX:+UseZGC -XX:-CreateCoredumpOnCrash \
                                      -XX:-OmitStackTraceInFastThrow -Xmx4g \
                                      -Djava.library.path=natives/;natives/win64/;.";
pub const VM_ARGS_32: &'static str = "-Djava.awt.headless=true -Xms768m -Xmx768m \
                                      -Dzomboid.steam=1 -Dzomboid.znetlog=1 \
                                      -Djava.library.path=natives/;natives/win32/;. -XX:+UseG1GC \
                                      -XX:-CreateCoredumpOnCrash -XX:-OmitStackTraceInFastThrow";
