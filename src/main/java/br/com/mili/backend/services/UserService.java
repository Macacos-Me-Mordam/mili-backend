package br.com.mili.backend.services;

import br.com.mili.backend.data.dto.UserDTO;
import static br.com.mili.backend.mapper.ObjectMapper.parseObject;
import static br.com.mili.backend.mapper.ObjectMapper.parseListObject;

import br.com.mili.backend.model.User;
import br.com.mili.backend.repository.UserRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;

@Service
public class UserService {

    private Logger logger = LoggerFactory.getLogger(UserService.class.getName());

    private final UserRepository repository;
    private final PasswordEncoder passwordEncoder;

    @Autowired
    public UserService(UserRepository repository, PasswordEncoder passwordEncoder) {
        this.repository = repository;
        this.passwordEncoder = passwordEncoder;
    }

    public UserDTO create(UserDTO user) {
        logger.info("Creating one user!");
        var entity = parseObject(user, User.class);
        entity.setPassword(passwordEncoder.encode(user.getPassword()));
        var newUser = repository.save(entity);
        return parseObject(newUser, UserDTO.class);
    }
}
