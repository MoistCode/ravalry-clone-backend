import {
  Args,
  Field,
  InputType,
  Int,
  Mutation,
  Query,
  Resolver
} from '@nestjs/graphql';
import { IsEmail, Length } from 'class-validator';

import { User } from './models/user.model';
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
    returns => User,
    {
      name: 'user',
      nullable: true,
    }
  )
  async getUserById(@Args('id', { type: () => Int }) id: number) {
    return this.usersService.findOneById(id);
  }

  @Mutation(returns => User)
  async createNewUser(@Args('newUserData') newUserData: CreateNewUserInput) {
    return this.usersService.createNewUser(newUserData);
  }
}