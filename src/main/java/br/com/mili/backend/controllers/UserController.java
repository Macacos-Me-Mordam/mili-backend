package br.com.mili.backend.controllers;

import br.com.mili.backend.data.dto.UserDTO;
import br.com.mili.backend.services.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;
import org.springframework.http.MediaType;

import java.util.List;

@RestController
@RequestMapping("/users")
public class UserController {

    @Autowired
    private UserService service;

    @PostMapping(consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    public UserDTO create(@RequestBody UserDTO user) {
        return service.create(user);
    }

    @GetMapping("/profile")
    public UserDTO getMe(Authentication authentication) {
        return service.getMe(authentication.getName());
    }

    @GetMapping
    public List<UserDTO> findAll() {
        return service.findAll();
    }
}
