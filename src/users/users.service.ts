import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';

import { UserEntity } from './user.entity';

type CreateUserInput = {
  email: string;
  firstName: string;
  lastName: string;
  password: string;
  username: string;
}

@Injectable()
export class UsersService {
  constructor(
    @InjectRepository(UserEntity)
    private usersRepository: Repository<UserEntity>,
  ) {}

  findOneById(id: number): Promise<UserEntity | null> {
    return this.usersRepository.findOneBy({ id });
  }

  /**
   * Creates a new user provided that the username and email does not exist in
   * the database.
   */
  createNewUser(newUserData: CreateUserInput): Promise<UserEntity> {
    const { email, firstName, lastName, password, username } = newUserData;

    const newUserEntity = this.usersRepository.create({
      email,
      firstName,
      lastName,
      salt: password,
      username,
    });

    return this.usersRepository.save(newUserEntity);
  }
}