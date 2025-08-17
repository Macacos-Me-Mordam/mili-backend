package br.com.mili.backend.data.dto;

import java.util.UUID;

public record UpdateOccurrenceStatusDTO(UUID id,
                                        String status) {
}
