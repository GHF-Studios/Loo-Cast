using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public interface ITransform : IComponent
    {
        #region Properties
        public IObjectType ObjectType { get; }
        public IObject ParentObject { get; }
        public List<IObject> ChildObjects { get; }
        public BigVector3 Position { get; set; }
        public Vector3? UnityPosition { get; set; }
        #endregion
    }
}