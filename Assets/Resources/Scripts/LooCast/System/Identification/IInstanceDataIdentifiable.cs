using System;

namespace LooCast.System.Identification
{
    public interface IInstanceDataIdentifiable : IDataIdentifiable
    {
        #region Properties
        IInstanceDataIdentifier InstanceDataIdentifier { get; }
        #endregion
    }
}
