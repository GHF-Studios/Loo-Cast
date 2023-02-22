using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IComponent : IGameObject, IComponentIdentifiable
    {
        #region Properties
        public IComponentType ComponentType { get; }
        public IComponent ParentComponent { get; }
        public List<IComponent> ChildComponents { get; }
        #endregion
    }
}
