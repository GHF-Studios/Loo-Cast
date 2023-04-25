using System.Collections.Generic;

namespace LooCast.System
{
    public class InstancePool<T> : IPool
        where T : IType.IInstance, new()
    {
        private readonly Stack<T> pool = new Stack<T>();

        object IPool.Get()
        {
            return Get();
        }

        public void Return(object obj)
        {
            pool.Push((T)obj);
        }
        
        public T Get()
        {
            return pool.Count > 0 ? pool.Pop() : new T();
        }

        public void Return(T obj)
        {
            pool.Push(obj);
        }
    }
}
