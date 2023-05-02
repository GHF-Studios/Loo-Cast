using System;

namespace LooCast.System
{
    public interface IIdentifiable
    {
        #region Properties
        IIdentifier Identifier { get; }
        #endregion
    }
}
