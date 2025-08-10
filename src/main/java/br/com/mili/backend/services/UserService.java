package br.com.mili.backend.services;

import br.com.mili.backend.data.dto.UserDTO;
import static br.com.mili.backend.mapper.ObjectMapper.parseObject;
import static br.com.mili.backend.mapper.ObjectMapper.parseListObject;

import br.com.mili.backend.model.User;
import br.com.mili.backend.repository.UserRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class UserService {

    private Logger logger = LoggerFactory.getLogger(UserService.class.getName());

    @Autowired
    UserRepository repository;

    public UserDTO create(UserDTO user) {
        logger.info("Creating one user!");
        var entity = parseObject(user, User.class);
        return parseObject(repository.save(entity), UserDTO.class);
    }
}
