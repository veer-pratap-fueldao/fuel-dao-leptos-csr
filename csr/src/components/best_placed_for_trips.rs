use leptos::*;
use leptos_icons::Icon;
use leptos_use::use_media_query;

#[component]
pub fn BestPlacedForTrips() -> impl IntoView {
    let items = vec![
        ("Villa Avenil", "/img/trips.svg", "$200 "),
        ("Villa Edo", "/img/trips.svg", "$180"),
        ("Villa Raffa", "/img/trips.svg", "$250 "),
        ("Villa Peony", "/img/trips.svg", "$300 "),
        ("Villa Amara", "/img/trips.svg", "$220 "),
        // ("Villa Avenil", "/img/2.jpg", "$200 / night"),
        ("Villa Edo", "/img/trips.svg", "$180 "),
        ("Villa Raffa", "/img/trips.svg", "$250"),
        ("Villa Peony", "/img/trips.svg", "$300 "),
        ("Villa Amara", "/img/trips.svg", "$220"),
    ];

    let length = items.len();
    let (current_index, set_current_index) = create_signal(2); // Start with the 3rd item (index 2)

    // Functions to rotate the carousel
    let rotate_left = move |_| {
        set_current_index.update(|index| *index = (*index + length - 1) % length);
    };

    let rotate_right = move |_| {
        set_current_index.update(|index| *index = (*index + 1) % length);
    };

    // Function to handle mouse click on images
    let handle_image_click = move |index: usize| {
        set_current_index.set(index);
    };
    let is_mobile = use_media_query("(max-width: 640px)").get();
    log::info!("Is Mobile: {}", is_mobile);

    // Calculate the 5 indices to be displayed in the carousel
    let get_display_count = move || {
        // Capture the media query and check if the screen width is less than or equal to 640px
        if use_media_query("(max-width: 640px)").get_untracked() {
            3 // Display 3 items on small screens
        } else {
            5 // Display 5 items on larger screens
        }
    };
    let log_size = {
        move || {
            logging::log!(
                "mobile screen: {:?}",
                use_media_query("(max-width: 640px)").get()
            )
        }
    };
    log_size();
    // Calculate the display indices to be displayed in the carousel
    let get_display_indices = |current: usize| {
        let count = get_display_count();
        (0..count)
            .map(|i| (current + i) % length) // Wrap around the array
            .collect::<Vec<_>>()
    };

    view! {
        <section class="py-12 bg-gray-100">
            <div class="container mx-auto text-center">
                <h2 class="mb-8 text-3xl font-bold">"Top spots near Banglore"</h2>
                <div class="flex relative justify-center items-center">

                    <div class="flex absolute left-0 items-center h-full">
                        <button on:click=rotate_left>
                            <Icon
                                class="bg-gray-300 rounded-full w-[24px] h-[24px] lg:w-[30px] lg:h-[30px]"
                                icon=icondata::BsArrowLeftCircle
                            />
                        </button>
                    </div>

                    <div class="flex overflow-hidden space-x-4 carousel">
                        {get_display_indices(current_index.get())
                            .into_iter()
                            .enumerate()
                            .map(move |(idx, i)| {
                                let is_first = idx == 0;
                                let is_last = idx == get_display_count() - 1;
                                let blur_class = if is_first || is_last {
                                    "filter blur-sm"
                                } else {
                                    ""
                                };
                                let scale = if is_first || is_last { 0.75 } else { 1.0 };
                                let transform_style = format!(
                                    "scale({}) translateX({}px)",
                                    scale,
                                    (idx as f32 - 1.0) * 20.0,
                                );
                                view! {
                                    // Adjusted for dynamic display count
                                    // Adjusted for dynamic index
                                    <div
                                        style=transform_style
                                        class=format!(
                                            "flex flex-col justify-center items-center carousel-item transition-transform duration-300 ease-in-out transform {}",
                                            if is_first || is_last {
                                                "w-[80%] sm:w-[90%] lg:w-auto"
                                            } else {
                                                "w-[80%] sm:w-[95%] lg:w-auto"
                                            },
                                        )
                                        // Handle mouse click
                                        on:click=move |_| handle_image_click(i)
                                    >
                                        <img
                                            src=items[i].1
                                            alt=items[i].0
                                            class=format!(
                                                "object-cover rounded-lg shadow-lg {}",
                                                if is_first || is_last {
                                                    " h-32  lg:h-[65%]"
                                                } else {
                                                    "h-40 lg:h-auto"
                                                },
                                            )
                                        />
                                        <div class=format!(
                                            "flex flex-col items-center mt-2 {}",
                                            if is_first || is_last { "w-[80%]" } else { "w-full" },
                                        )>

                                            <div class="flex justify-between w-full">
                                                <h3 class="text-lg font-bold text-center">{items[i].0}</h3>
                                                <div class="flex flex-col items-end">
                                                    <p class="font-bold text-green-500">{items[i].2}</p>
                                                    <p class="text-sm text-gray-500">{"per month"}</p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>

                    <div class="flex absolute right-0 items-center h-full">
                        <button on:click=rotate_right>
                            <Icon
                                class="bg-gray-300 rounded-full w-[24px] h-[24px] lg:w-[30px] lg:h-[30px]"
                                icon=icondata::BsArrowRightCircle
                            />
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
