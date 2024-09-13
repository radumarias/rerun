// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/video_frame_reference.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/entity_path.hpp"
#include "../components/video_timestamp.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: References a single video frame.
    ///
    /// Used to display individual video frames from a `archetypes::AssetVideo`.
    /// To show an entire video, a fideo frame reference for each frame of the video should be logged.
    ///
    /// ## Example
    ///
    /// ### Video with explicit frames
    /// ![image](https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <iostream>
    ///
    /// using namespace std::chrono_literals;
    ///
    /// int main(int argc, char* argv[]) {
    ///     if (argc <2) {
    ///         // TODO(#7354): Only mp4 is supported for now.
    ///         std::cerr <<"Usage: " <<argv[0] <<" <path_to_video.[mp4]>" <<std::endl;
    ///         return 1;
    ///     }
    ///
    ///     const auto path = argv[1];
    ///
    ///     const auto rec = rerun::RecordingStream("rerun_example_asset_video_manual_frames");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Log video asset which is referred to by frame references.
    ///     // Make sure it's available on the timeline used for the frame references.
    ///     rec.set_time_seconds("video_time", 0.0);
    ///     rec.log("video", rerun::AssetVideo::from_file(path).value_or_throw());
    ///
    ///     // Send frame references for every 0.1 seconds over a total of 10 seconds.
    ///     // Naturally, this will result in a choppy playback and only makes sense if the video is 10 seconds or longer.
    ///     // TODO(#7368): Point to example using `send_video_frames`.
    ///     //
    ///     // Use `send_columns` to send all frame references in a single call.
    ///     std::vector<std::chrono::milliseconds> times(10 * 10);
    ///     std::vector<rerun::components::VideoTimestamp> video_timestamps(10 * 10);
    ///     for (size_t i = 0; i <times.size(); i++) {
    ///         times[i] = 100ms * i;
    ///         video_timestamps[i] = rerun::components::VideoTimestamp(times[i]);
    ///     }
    ///     auto video_frame_reference_indicators =
    ///         rerun::ComponentColumn::from_indicators<rerun::VideoFrameReference>(
    ///             static_cast<uint32_t>(times.size())
    ///         );
    ///     rec.send_columns(
    ///         "video",
    ///         rerun::TimeColumn::from_times("video_time", rerun::borrow(times)),
    ///         {
    ///             video_frame_reference_indicators.value_or_throw(),
    ///             rerun::ComponentColumn::from_loggable(rerun::borrow(video_timestamps)).value_or_throw(),
    ///         }
    ///     );
    /// }
    /// ```
    ///
    /// ⚠ **This is an experimental API! It is not fully supported, and is likely to change significantly in future versions.**
    struct VideoFrameReference {
        /// References the closest video frame to this timestamp.
        ///
        /// Note that this uses the closest video frame instead of the latest at this timestamp
        /// in order to be more forgiving of rounding errors for inprecise timestamp types.
        rerun::components::VideoTimestamp timestamp;

        /// Optional reference to an entity with a `archetypes::AssetVideo`.
        ///
        /// If none is specified, the video is assumed to be at the same entity.
        /// Note that blueprint overrides on the referenced video will be ignored regardless,
        /// as this is always interpreted as a reference to the data store.
        ///
        /// For a series of video frame references, it is recommended to specify this path only once
        /// at the beginning of the series and then rely on latest-at query semantics to
        /// keep the video reference active.
        std::optional<rerun::components::EntityPath> video_reference;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.VideoFrameReferenceIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        VideoFrameReference() = default;
        VideoFrameReference(VideoFrameReference&& other) = default;

        explicit VideoFrameReference(rerun::components::VideoTimestamp _timestamp)
            : timestamp(std::move(_timestamp)) {}

        /// Optional reference to an entity with a `archetypes::AssetVideo`.
        ///
        /// If none is specified, the video is assumed to be at the same entity.
        /// Note that blueprint overrides on the referenced video will be ignored regardless,
        /// as this is always interpreted as a reference to the data store.
        ///
        /// For a series of video frame references, it is recommended to specify this path only once
        /// at the beginning of the series and then rely on latest-at query semantics to
        /// keep the video reference active.
        VideoFrameReference with_video_reference(rerun::components::EntityPath _video_reference
        ) && {
            video_reference = std::move(_video_reference);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::VideoFrameReference> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const archetypes::VideoFrameReference& archetype
        );
    };
} // namespace rerun
