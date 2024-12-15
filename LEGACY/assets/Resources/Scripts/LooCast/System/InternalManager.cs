using LooCast.System.Identifiers;
using System;

namespace LooCast.System
{
    public abstract class InternalManager : Manager
    {
        protected InternalManager(TypeIdentifier typeIdentifier, GameObject parentGameObject = null) : base(typeIdentifier, parentGameObject)
        {
            
        }
    }
}
