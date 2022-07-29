import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo';
import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { JwtService } from '@nestjs/jwt';
import { GraphQLModule } from '@nestjs/graphql';
import { TypeOrmModule } from '@nestjs/typeorm';
import { join } from 'path';
import { DataSource } from 'typeorm';

import { AuthModule } from './auth/auth.module';
import { AuthService } from './auth/auth.service';
import { UsersModule } from './users/users.module';

@Module({
  imports: [
    ConfigModule.forRoot(),

    GraphQLModule.forRoot<ApolloDriverConfig>({
      driver: ApolloDriver,
      autoSchemaFile: join(process.cwd(), 'src/schema.gql'),
      sortSchema: true,
    }),

    TypeOrmModule.forRootAsync({
      imports: [ConfigModule],
      inject: [ConfigService],
      useFactory: (configService: ConfigService) => {
        return {
          type: 'postgres',
          port: Number(configService.get<string>('POSTGRESQL_PORT')),
          username: configService.get<string>('POSTGRESQL_USERNAME'),
          password: configService.get<string|undefined>('POSTGRESQL_PASSWORD'),
          database: configService.get<string>('POSTGRESQL_DATABASE'),
          // @TODO Disable synchronize in production.
          synchronize: true,
          autoLoadEntities: true,
        };
      }
    }),

    AuthModule,
    UsersModule
  ],

  providers: [AuthService, JwtService],
})

export class AppModule {
  constructor(private dataSource: DataSource) {}
}
