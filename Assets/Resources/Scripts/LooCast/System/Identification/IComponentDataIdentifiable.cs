using System;

namespace LooCast.System.Identification
{
    public interface IComponentDataIdentifiable : IGameObjectDataIdentifiable
    {
        #region Properties
        IComponentDataIdentifier ComponentDataIdentifier { get; }
        #endregion
    }
}
