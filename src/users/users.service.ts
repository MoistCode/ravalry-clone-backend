import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import bcrypt from 'bcryptjs';
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

  findOneByUsername(username: string): Promise<UserEntity | null> {
    return this.usersRepository.findOneBy({ username });
  }

  /**
   * Creates a new user provided that the username and email does not exist in
   * the database.
   */
  async createNewUser(newUserData: CreateUserInput) {
    const { email, firstName, lastName, password, username } = newUserData;

    const salt = bcrypt.genSaltSync(10);
    const hash = bcrypt.hashSync(password, salt);

    const newUserEntity = this.usersRepository.create({
      email,
      firstName,
      lastName,
      hash,
      username,
    });

    const {
      hash: removedHash,
      ...result
    } = await this.usersRepository.save(newUserEntity);

    return result;
  }
}