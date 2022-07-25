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

  createNewUser(newUserData: CreateUserInput): Promise<UserEntity> {
    const newUserEntity = this.usersRepository.create({
      email: newUserData.email,
      firstName: newUserData.firstName,
      lastName: newUserData.lastName,
      salt: newUserData.password,
      username: newUserData.username,
    });

    return this.usersRepository.save(newUserEntity);
  }
}