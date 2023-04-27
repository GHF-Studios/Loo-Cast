using System;

namespace LooCast.System
{
    public interface IPool<T>
    {
        #region Methods
        public T Get();
        public void Return(T obj);
        #endregion
    }
}
