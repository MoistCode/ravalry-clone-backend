import { Int, Field, ObjectType } from '@nestjs/graphql';
import { IsEmail, Length } from 'class-validator';

@ObjectType()
export class User {
  @Field(type => Int)
  id: number;

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
  @Length(5, 255)
  username: string;
};