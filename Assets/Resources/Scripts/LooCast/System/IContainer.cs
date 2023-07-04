using System.Collections.Generic;

namespace LooCast.System
{
    public interface IContainer<TObject>
    {
        #region Properties
        IEnumerable<TObject> Objects { get; }
        #endregion
    }
}
