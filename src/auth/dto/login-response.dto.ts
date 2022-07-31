import { Field, ObjectType } from '@nestjs/graphql';
import { UserModel } from '../../users/models/user.model';

@ObjectType()
export class LoginResponse {
  @Field()
  access_token: string;

  @Field(() => UserModel)
  user: UserModel;
}