using System.Collections.Generic;

namespace LooCast.System
{
    public class ObjectPool<T> where T : new()
    {
        private readonly Stack<T> pool = new Stack<T>();

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
