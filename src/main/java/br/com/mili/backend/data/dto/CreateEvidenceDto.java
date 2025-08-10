package br.com.mili.backend.data.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;

import java.util.UUID;

public record CreateEvidenceDto(
        @NotBlank String description,
        @JsonProperty("file_path") @NotBlank String filePath,
        @JsonProperty("camera_id") @NotNull UUID cameraId
) {}