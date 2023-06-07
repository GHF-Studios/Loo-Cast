using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IParent<TChild>
    {
        #region Properties
        IEnumerable<TChild> Children { get; }
        #endregion
    }
}
