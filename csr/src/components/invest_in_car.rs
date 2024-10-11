use leptos::*;

#[component]
pub fn InvestInCar() -> impl IntoView {
    view! {
        <section class="relative w-full h-[722.64px] lg:top-[0px] lg:w-[1440px]">
            <div
                class="absolute w-full lg:w-[1145.19px] lg:h-[489.37px] lg:left-[147.4px] lg:top-12 lg:pt-12 border-[2px] border-solid border-[#03B74B] rounded-[20px] flex flex-col lg:flex-row  lg:p-12"
                style="gap: 121px;"
            >
                <div class="flex gap-0 justify-center items-center h-[393.37px] lg:w-[443.63px]">
                    <img
                        src="/img/Group.png"
                        alt="Left Image"
                        class="object-cover w-full h-full rounded-[20px]"
                    />
                </div>

                <div class="flex flex-col justify-center p-4 opacity-100 lg:p-0 lg:h-[326.78px] lg:w-[484.56px] lg:gap-[40px]">
                    <div class="flex gap-0 items-center lg:h-[144.81px] lg:w-[484.56px]">
                        <p class="font-bold leading-none text-left font-baloo text-[75.41px]">
                            I<span style="color: #00b84c;">n</span>vest in a <br />Car
                            N<span style="color: #00b84c;">e</span>twork
                        </p>
                    </div>

                    <div class="h-auto gap-[9.99px] lg:w-[484.56px]">
                        <p class="font-normal text-left font-lato text-[18px] leading-[21.6px]">
                            If you need consultation with us, you can write a message or call us, we will respond as quickly as possible
                        </p>
                    </div>

                    <div class="flex">
                        <button class="w-[164.94px] lg:h-[57.96px] p-[9.99px_24.98px] gap-[9.99px] border-[2px] border-solid border-[#03B74B] bg-[#03B74B] text-white rounded-tl-[3px] rounded-tr-none rounded-br-none rounded-bl-none text-left">
                            Coming Soon
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
