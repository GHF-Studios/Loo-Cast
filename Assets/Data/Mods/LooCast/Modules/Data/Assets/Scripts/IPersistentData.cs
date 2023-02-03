using System;

namespace LooCast.Data
{
    public interface IPersistentData : IData
    {
        #region Properties
        DataFolder ParentFolder { get; }
        #endregion
    }
}
