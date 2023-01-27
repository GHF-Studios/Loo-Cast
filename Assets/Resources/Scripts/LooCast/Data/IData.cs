using System;

namespace LooCast.Data
{
    public interface IData
    {
        #region Properties
        string ID { get; }
        Type DataType { get; }
        string Name { get; }
        #endregion

        #region Methods

        #endregion
    }
}
