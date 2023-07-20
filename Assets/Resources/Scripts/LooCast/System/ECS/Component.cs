using System;
using System.Collections.Generic;

namespace LooCast.System.ECS
{
    public abstract class Component : IComponent
    {
        #region Properties
        public Guid ComponentID { get; private set; }
        public IEntity Entity { get; private set; }
        #endregion

        #region Constructors
        protected Component()
        {
            ComponentID = Guid.NewGuid();
        }
        #endregion

        #region Callbacks
        public virtual void OnCreate()
        {

        }

        public virtual void OnDestroy()
        {

        }
        #endregion

        #region Methods
        public void Initialize_INTERNAL(IEntity entity)
        {
            if (Entity != null)
            {
                throw new InvalidOperationException("Component has already been initialized!");
            }

            Entity = entity;
        }

        public void Destroy_INTERNAL()
        {
            ComponentID = Guid.Empty;
            Entity = null;
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return ComponentID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not Component)
            {
                return false;
            }

            Component other = (Component)obj;
            return other.ComponentID == this.ComponentID;
        }

        public override string ToString()
        {
            return ComponentID.ToString();
        }
        #endregion
    }
}
