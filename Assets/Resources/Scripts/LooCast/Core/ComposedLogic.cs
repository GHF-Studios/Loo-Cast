using System;
using System.Collections.Generic;

namespace LooCast.Core
{
    using LooCast.System;

    public abstract class ComposedLogic : ILogic, IParent<ILogic>
    {
        #region Properties
        public Guid LogicID { get; private set; }
        IEnumerable<ILogic> IParent<ILogic>.Children => LogicChildren;
        public IEnumerable<ILogic> LogicChildren => logicChildrenList;
        #endregion

        #region Fields
        private List<ILogic> logicChildrenList;
        #endregion

        #region Constructors
        protected ComposedLogic()
        {
            LogicID = new Guid();
            logicChildrenList = new List<ILogic>();
        }
        #endregion

        #region Methods

        public virtual bool Validate()
        {
            return true;
        }

        public virtual bool TryAddChildLogic(ILogic childLogic)
        {
            if (ContainsChildLogic(childLogic.LogicID))
            {
                return false;
            }
            else
            {
                AddChildLogic(childLogic);
                return true;
            }
        }
        public virtual void AddChildLogic(ILogic childLogic)
        {
            if (ContainsChildLogic(childLogic))
            {
                throw new InvalidOperationException($"ComposedLogic '{this}' already contains Logic '{childLogic}'!");
            }
            logicChildrenList.Add(childLogic);
        }

        public virtual bool TryRemoveChildLogic(ILogic childLogic)
        {
            if (!ContainsChildLogic(childLogic))
            {
                return false;
            }
            else
            {
                RemoveChildLogic(childLogic);
                return true;
            }
        }
        public virtual void RemoveChildLogic(ILogic childLogic)
        {
            logicChildrenList.Remove(childLogic);
        }

        public virtual bool TryGetChildLogic(Guid childLogicID, out ILogic childLogic)
        {
            if (!ContainsChildLogic(childLogicID))
            {
                childLogic = null;
                return false;
            }
            else
            {
                childLogic = GetChildLogic(childLogicID);
                return true;
            }
        }
        public virtual ILogic GetChildLogic(Guid childLogicID)
        {
            return logicChildrenList.Find((logicChild) => { return logicChild.LogicID == childLogicID; });
        }
        public virtual bool ContainsChildLogic(Guid childLogicID)
        {
            return logicChildrenList.Exists((childLogic) => { return childLogic.LogicID == childLogicID; });
        }
        public virtual bool ContainsChildLogic(ILogic childLogic)
        {
            return logicChildrenList.Contains(childLogic);
        }

        public virtual void ClearChildLogics()
        {
            logicChildrenList.Clear();
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return LogicID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not ComposedLogic)
            {
                return false;
            }

            ComposedLogic other = (ComposedLogic)obj;
            return other.LogicID == this.LogicID;
        }

        public override string ToString()
        {
            return LogicID.ToString();
        }
        #endregion
    }
}
