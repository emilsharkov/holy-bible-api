import { UserModel, IUser } from '../models/user.model';

export const getUserById = async (id: string): Promise<IUser | null> => {
  return await UserModel.findById(id);
};

export const createUser = async (data: IUser): Promise<IUser> => {
  const user = new UserModel(data);
  return await user.save();
};