using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IChild<TParent>
    {
        #region Properties
        TParent Parent { get; }
        #endregion

        #region Methods
        void SetParent(TParent parent);
        #endregion
    }
}
