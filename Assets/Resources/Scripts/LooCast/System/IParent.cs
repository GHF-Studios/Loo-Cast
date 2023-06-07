using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IParent<TChild>
    {
        #region Properties
        IEnumerable<TChild> Children { get; }
        #endregion

        #region Methods
        void AddChild(TChild child);
        bool RemoveChild(TChild child);
        bool ContainsChild(TChild child);
        void ClearChildren();
        #endregion
    }
}
