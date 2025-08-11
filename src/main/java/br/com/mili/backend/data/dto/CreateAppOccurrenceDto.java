package br.com.mili.backend.data.dto;

import br.com.mili.backend.data.enums.FrequencyTypeEnum;
import com.fasterxml.jackson.annotation.JsonProperty;
import jakarta.validation.constraints.NotBlank;

public record CreateAppOccurrenceDto(
        @NotBlank String description,
        @JsonProperty("photo_url") @NotBlank String photoUrl,
        @NotBlank String address,
        @NotBlank FrequencyTypeEnum frequency
) {
}
