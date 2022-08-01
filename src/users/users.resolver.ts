import {
  Args,
  Int,
  Mutation,
  Query,
  Resolver,
} from '@nestjs/graphql';

import { CreateUserInput } from './dto/create-user.input';
import { UserModel } from './models/user.model';
import { UsersService } from './users.service';

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
  async createNewUser(@Args('newUserData') newUserData: CreateUserInput) {
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