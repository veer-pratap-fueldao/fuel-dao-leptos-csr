use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn GetInTouch() -> impl IntoView {
    view! {
        <section class="flex relative flex-col pt-40 lg:flex-row lg:pt-0 lg:m-32 gap-[137.87px] lg:w-[1238.88px] lg:h-[567.6px]">

            <div class="flex flex-col gap-[64.94px] h-[567.6px] lg:w-[491.55px]">
                <div class="flex flex-col gap-[24.98px] h-[452.7px] lg:w-[491.55px]">
                    <div class="flex flex-col w-full h-[177.87px] lg:w-[491.55px]">
                        <div class="flex flex-col w-[229.98px] h-[121.89px]">
                            <h3 class="w-[158.98px] h-[43.98px] pt-[9.99px] text-[#03B74B] ">
                                GET IN TOUCH
                            </h3>
                            <h1 class="text-4xl w-[229.98px] h-[71.98px] pt-[9.99px]">
                                Contact Us
                            </h1>
                        </div>
                        <div class="flex gap-0 w-full h-[55.98px] pt-[9.99px] lg:w-[484.56px]">
                            <p class="overflow-hidden font-normal text-left break-words font-lato text-[14.99px] leading-[17.98px]">
                                If you need consultation with us, you can write a message or call us, we will respond as quickly as possible.
                            </p>
                        </div>
                    </div>
                    <div class="flex flex-col gap-0 w-[339.92px] h-[249.85px]">
                        <div class="flex flex-row items-center gap-[9.99px] w-[253.92px] h-[57.96px]">
                            <button class="flex items-center">
                                <Icon
                                    class="w-[24px] h-[24px] text-[#03B74B]"
                                    icon=icondata::FiMail
                                />
                            </button>
                            <p class="pl-2 m-0">fueldao@gmail.com</p>
                        </div>

                        <div class="flex flex-row items-center gap-[9.99px] w-[253.92px] h-[57.96px]">
                            <button class="flex items-center">
                                <Icon
                                    class="w-[24px] h-[24px] text-[#03B74B]"
                                    icon=icondata::FiPhone
                                />
                            </button>
                            <p class="pl-2 m-0">+62 8221 1222 0001</p>
                        </div>

                        <div class="flex flex-row items-center gap-[9.99px] w-[253.92px] h-[57.96px]">
                            <button class="flex items-center">
                                <Icon
                                    class="w-[26px] h-[26px] text-[#03B74B]"
                                    icon=icondata::WiTime4
                                />
                            </button>
                            <p class="pl-2 m-0">Everyday : 08.00-21.00</p>
                        </div>

                        <div class="flex flex-row items-center gap-[9.99px] w-[253.92px] h-[57.96px]">
                            <button class="flex items-center">
                                <Icon
                                    class="w-[26px] h-[26px] text-[#03B74B]"
                                    icon=icondata::TiLocationOutline
                                />
                            </button>
                            <p class="pl-2 m-0">
                                103, Office , Main  road Selatan,
                                India 41222
                            </p>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col gap-[9.99px] w-[184.83px] h-[49.95px] pt-[9.99px]">
                    <div class="flex flex-row gap-[14.99px] w-[164.85px] h-[29.97px]">
                        <button class="border-t border-transparent icon w-[29.97px] h-[29.97px]">
                            <Icon
                                icon=icondata::BiLinkedinSquare
                                class="w-full h-full text-[#03B74B]"
                            />
                        </button>
                        <button class="border-t border-transparent icon w-[29.97px] h-[29.97px]">
                            <Icon icon=icondata::BsTwitterX class="w-full h-full text-[#03B74B]" />
                        </button>
                        <button class="border-t border-transparent icon w-[29.97px] h-[29.97px]">
                            <Icon icon=icondata::LuSend class="w-full h-full text-[#03B74B]" />
                        </button>
                        <button class="border-t border-transparent icon w-[29.97px] h-[29.97px]">
                            <Icon icon=icondata::BiInstagram class="w-full h-full text-[#03B74B]" />
                        </button>
                    </div>
                </div>
            </div>
            <div class="flex flex-col">
                <div class="flex m-8 lg:m-0 h-[446.59px] lg:gap-[49.95px] lg:w-[609.45px]">

                    <iframe
                        src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3164.28354119872!2d-122.0842497!3d37.4220653!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x808fb5aaf1c3f15d%3A0xa44bc518fdb3f92a!2sGoogleplex!5e0!3m2!1sen!2sin!4v1549380919454"
                        width="100%"
                        height="100%"
                        style="border:0;"
                        allowfullscreen=""
                        loading="lazy"
                    ></iframe>

                </div>

            </div>
        </section>
    }
}
