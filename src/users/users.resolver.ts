import {
  Args,
  Field,
  InputType,
  Int,
  Mutation,
  Query,
  Resolver,
} from '@nestjs/graphql';
import { IsEmail, Length } from 'class-validator';

import { UserModel } from './models/user.model';
import { UsersService } from './users.service';

@InputType()
export class CreateNewUserInput {
  @Field()
  @IsEmail()
  email: string;

  @Field()
  @Length(1, 255)
  firstName: string;

  @Field()
  @Length(1, 255)
  lastName: string;

  @Field()
  @Length(8, 255)
  password: string;

  @Field()
  @Length(5, 255)
  username: string;
}

@Resolver()
export class UsersResolver {
  constructor(
    private usersService: UsersService
  ) {}

  @Query(
    returns => UserModel,
    {
      name: 'user',
      nullable: true,
    }
  )
  async getUserById(@Args('id', { type: () => Int }) id: number) {
    return this.usersService.findOneById(id);
  }

  @Mutation(returns => UserModel)
  async createNewUser(@Args('newUserData') newUserData: CreateNewUserInput) {
    try {
      return await this.usersService.createNewUser(newUserData);
    } catch (e: any) {
      switch(e.name) {
        case 'QueryFailedError': {
          throw new Error('User already exists.');
        }

        default: {
          throw new Error('Something went wrong when trying to create a new user.');
        }
      }
    }
  }
}