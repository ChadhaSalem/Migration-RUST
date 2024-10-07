cmake_minimum_required(VERSION 3.10)
project(cross_lib C)
# Définir l'outil de compilation croisée
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR arm)
# Spécifier le compilateur C
set(CMAKE_C_COMPILER arm-none-eabi-gcc)
# Ajouter des options de compilation spécifiques
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=hard")
# Ajouter des options de liaison spécifiques
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=hard")
# Ajouter des répertoires d'inclusion
include_directories(${CMAKE_SOURCE_DIR})
# Ajouter les fichiers source C
set(SOURCES
    code.c
)
# Ajouter la bibliothèque Rust
add_library(cross_lib STATIC IMPORTED)
set_target_properties(cross_lib PROPERTIES IMPORTED_LOCATION
    ${CMAKE_SOURCE_DIR}/../target/thumbv7em-none-eabihf/release/libcross_lib.a
)
# Ajouter l'exécutable
add_executable(main ${SOURCES})
# Lier l'exécutable avec la bibliothèque Rust
target_link_libraries(main cross_lib)
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,--no-accept-unknown-input-arch")
add_link_options(--major-image-version)
