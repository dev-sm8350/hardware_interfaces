/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
mod default_audio_control_hal;

use android_hardware_automotive_audiocontrol::aidl::android::hardware::automotive::audiocontrol::IAudioControl::BnAudioControl;
use crate::default_audio_control_hal::DefaultAudioControlHal;

fn main() {
    binder::ProcessState::start_thread_pool();
    let my_service = DefaultAudioControlHal;
    let service_name = "android.hardware.automotive.audiocontrol.IAudioControl/default";
    let my_service_binder = BnAudioControl::new_binder(
        my_service,
        binder::BinderFeatures::default(),
    );
    binder::add_service(service_name, my_service_binder.as_binder())
    		.expect(format!("Failed to register {}?", service_name).as_str());
    // Does not return.
    binder::ProcessState::join_thread_pool()
}