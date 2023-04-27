using System;
using System.Collections.Generic;

namespace LooCast.System.Identifiers
{
    public interface IGameObjectIdentifier : IInstanceIdentifier
    {
        #region Properties
        public TypeIdentifier GameObjectTypeIdentifier { get; }
        public Guid GameObjectInstanceGUID { get; }
        #endregion
    }
}
