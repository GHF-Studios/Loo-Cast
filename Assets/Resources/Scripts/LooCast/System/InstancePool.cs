using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Types;
    
    public class InstancePool<InstanceType> : IPool
        where InstanceType : IInstanceType.IInstance, new()
    {
        private readonly Stack<InstanceType> pool = new Stack<InstanceType>();

        object IPool.Get()
        {
            return Get();
        }

        public void Return(object obj)
        {
            pool.Push((InstanceType)obj);
        }
        
        public InstanceType Get()
        {
            return pool.Count > 0 ? pool.Pop() : new InstanceType();
        }

        public void Return(InstanceType obj)
        {
            pool.Push(obj);
        }
    }
}
