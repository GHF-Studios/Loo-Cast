using System;

namespace LooCast.System.Identification
{
    public interface IComponentMetaDataIdentifiable : IGameObjectMetaDataIdentifiable
    {
        #region Properties
        IComponentMetaDataIdentifiable ComponentMetaDataIdentifier { get; }
        #endregion
    }
}
