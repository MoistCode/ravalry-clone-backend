import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import bcrypt from 'bcryptjs';
import { Repository } from 'typeorm';

import { CreateUserInput } from './dto/create-user.input';
import { UserEntity } from './user.entity';

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

  findOneByUsernameWithHash(username: string) {
    return this.usersRepository
      .createQueryBuilder('user')
      .where('user.username = :username', { username })
      .addSelect('user.hash')
      .getOneOrFail();
  }

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