#include "mlx-cxx/mlx_cxx.hpp"
#include "mlx-cxx/array.hpp"
#include "mlx-cxx/types.hpp"

#include "mlx-sys/src/types/float16.rs.h"
#include "mlx-sys/src/types/bfloat16.rs.h"
#include "mlx-sys/src/types/complex64.rs.h"

#include "mlx/types/half_types.h"
#include "mlx/types/complex.h"
#include "mlx/array.h"

namespace mlx_cxx {
    // std::unique_ptr<array> array_new_bool(bool value) {
    //     return mlx_cxx::new_unique<array>(value);
    // }

    // std::unique_ptr<array> array_new_f16(float16_t value) {
    //     mlx::core::float16_t value2 = mlx_cxx::f16_to_float16_t(value);
    //     return mlx_cxx::new_unique<array>(value2);
    // }

    // std::unique_ptr<array> array_new_bf16(bfloat16_t value) {
    //     mlx::core::bfloat16_t value2 = mlx_cxx::bf16_to_bfloat16_t(value);
    //     return mlx_cxx::new_unique<array>(value2);
    // }

    // std::unique_ptr<array> array_new_c64(complex64_t value) {
    //     return mlx_cxx::new_unique<array>(value);
    // }

    bool array_item_bool(array& arr, bool retain_graph) {
        return arr.item<bool>(retain_graph);
    }
}
