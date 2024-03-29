#include "mlx-cxx/fast.hpp"
#include "mlx-cxx/utils.hpp"

namespace mlx_cxx::fast {
        std::unique_ptr<mlx::core::array> rope(
        const mlx::core::array& x,
        int dims,
        bool traditional,
        float base,
        float scale,
        int offset,
        mlx_cxx::StreamOrDevice s)
    {
        auto arr = mlx::core::fast::rope(
            x,
            dims,
            traditional,
            base,
            scale,
            offset,
            s.to_variant());
        return std::make_unique<mlx::core::array>(arr);
    }

    std::unique_ptr<mlx::core::array> scaled_dot_product_attention(
        const mlx::core::array & queries,
        const mlx::core::array & keys,
        const mlx::core::array & values,
        const float scale,
        const mlx_cxx::OptionalArray & mask,
        mlx_cxx::StreamOrDevice s
    ) {
        auto std_mask = mlx_cxx::to_std_optional(mask);

        auto arr = mlx::core::fast::scaled_dot_product_attention(
            queries,
            keys,
            values,
            scale,
            std_mask,
            s.to_variant());

        return std::make_unique<mlx::core::array>(arr);
    }
    

    std::unique_ptr<mlx::core::array> rms_norm(
        const mlx::core::array & x,
        const mlx::core::array & weight,
        float eps,
        mlx_cxx::StreamOrDevice s
    )
    {
        auto arr = mlx::core::fast::rms_norm(
            x,
            weight,
            eps,
            s.to_variant());
        return std::make_unique<mlx::core::array>(arr);
    }

    std::unique_ptr<mlx::core::array> layer_norm(
        const mlx::core::array & x,
        const OptionalArray & weight,
        const OptionalArray & bias,
        float eps,
        mlx_cxx::StreamOrDevice s
    )
    {
        auto std_weight = mlx_cxx::to_std_optional(weight);
        auto std_bias = mlx_cxx::to_std_optional(bias);

        auto arr = mlx::core::fast::layer_norm(
            x,
            std_weight,
            std_bias,
            eps,
            s.to_variant());
        return std::make_unique<mlx::core::array>(arr);
    }
}