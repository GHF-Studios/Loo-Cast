using System;
using System.Collections.Generic;

namespace LooCast.Core
{
    using LooCast.System;

    public abstract class ComposedData : IData, IParent<IData>
    {
        #region Properties
        public Guid DataID { get; private set; }
        IEnumerable<IData> IParent<IData>.Children => DataChildren;
        public IEnumerable<IData> DataChildren => dataChildrenList;
        #endregion

        #region Fields
        private List<IData> dataChildrenList;
        #endregion

        #region Constructors
        protected ComposedData()
        {
            DataID = new Guid();
            dataChildrenList = new List<IData>();
        }
        #endregion

        #region Methods

        public virtual bool Validate()
        {
            return true;
        }

        public virtual bool TryAddChildData(IData childData)
        {
            if (ContainsChildData(childData.DataID))
            {
                return false;
            }
            else
            {
                AddChildData(childData);
                return true;
            }
        }
        public virtual void AddChildData(IData childData)
        {
            if (ContainsChildData(childData))
            {
                throw new InvalidOperationException($"ComposedData '{this}' already contains Data '{childData}'!");
            }
            dataChildrenList.Add(childData);
        }

        public virtual bool TryRemoveChildData(IData childData)
        {
            if (!ContainsChildData(childData))
            {
                return false;
            }
            else
            {
                RemoveChildData(childData);
                return true;
            }
        }
        public virtual void RemoveChildData(IData childData)
        {
            dataChildrenList.Remove(childData);
        }

        public virtual bool TryGetChildData(Guid childDataID, out IData childData)
        {
            if (!ContainsChildData(childDataID))
            {
                childData = null;
                return false;
            }
            else
            {
                childData = GetChildData(childDataID);
                return true;
            }
        }
        public virtual IData GetChildData(Guid childDataID)
        {
            return dataChildrenList.Find((dataChild) => { return dataChild.DataID == childDataID; });
        }
        public virtual bool ContainsChildData(Guid childDataID)
        {
            return dataChildrenList.Exists((childData) => { return childData.DataID == childDataID; });
        }
        public virtual bool ContainsChildData(IData childData)
        {
            return dataChildrenList.Contains(childData);
        }

        public virtual void ClearChildDatas()
        {
            dataChildrenList.Clear();
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return DataID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not ComposedData)
            {
                return false;
            }

            ComposedData other = (ComposedData)obj;
            return other.DataID == this.DataID;
        }

        public override string ToString()
        {
            return DataID.ToString();
        }
        #endregion
    }
}
