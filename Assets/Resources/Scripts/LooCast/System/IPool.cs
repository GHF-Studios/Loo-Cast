using System;

namespace LooCast.System
{
    public interface IPool
    {
        #region Methods
        public object Get();
        public void Return(object obj);
        #endregion
    }
}
