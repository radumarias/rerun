// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/material.fbs".

#include "material.hpp"

#include "../arrow.hpp"
#include "../datatypes/material.hpp"

#include <arrow/builder.h>
#include <arrow/table.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char Material::NAME[] = "rerun.components.Material";

        const std::shared_ptr<arrow::DataType> &Material::arrow_datatype() {
            static const auto datatype = rerun::datatypes::Material::arrow_datatype();
            return datatype;
        }

        Result<std::shared_ptr<arrow::StructBuilder>> Material::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(rerun::datatypes::Material::new_arrow_array_builder(memory_pool).value);
        }

        Error Material::fill_arrow_array_builder(
            arrow::StructBuilder *builder, const Material *elements, size_t num_elements
        ) {
            if (!builder) {
                return Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
            }
            if (!elements) {
                return Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            static_assert(sizeof(rerun::datatypes::Material) == sizeof(Material));
            RR_RETURN_NOT_OK(rerun::datatypes::Material::fill_arrow_array_builder(
                builder,
                reinterpret_cast<const rerun::datatypes::Material *>(elements),
                num_elements
            ));

            return Error::ok();
        }

        Result<rerun::DataCell> Material::to_data_cell(
            const Material *instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool *pool = arrow::default_memory_pool();

            auto builder_result = Material::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(
                    Material::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema =
                arrow::schema({arrow::field(Material::NAME, Material::arrow_datatype(), false)});

            rerun::DataCell cell;
            cell.component_name = Material::NAME;
            const auto ipc_result = rerun::ipc_from_table(*arrow::Table::Make(schema, {array}));
            RR_RETURN_NOT_OK(ipc_result.error);
            cell.buffer = std::move(ipc_result.value);

            return cell;
        }
    } // namespace components
} // namespace rerun