using System;

namespace LooCast.Registry
{
    public interface IRegistry<T>
    {
        string ID { get; }
        void Register(T item);
        void Unregister(T item);
        T Get(string id);
        bool Contains(string id);
    }
}
