namespace LooCast.StateMachine
{
    public class State<T>
    {
        public T ID { get; private set; }
        public string Name { get; private set; }

        public delegate void DelegateNoArg();
        public DelegateNoArg OnEnter;
        public DelegateNoArg OnExit;
        public DelegateNoArg OnUpdate;
        public DelegateNoArg OnFixedUpdate;

        public State(T id)
        {
            ID = id;
        }

        public State(T id, string name) : this(id)
        {
            Name = name;
        }

        public State
        (
            T id, 
            DelegateNoArg onEnter, 
            DelegateNoArg onExit = null, 
            DelegateNoArg onUpdate = null, 
            DelegateNoArg onFixedUpdate = null 
        ) : this(id)
        {
            OnEnter = onEnter;
            OnExit = onExit;
            OnUpdate = onUpdate;
            OnFixedUpdate = onFixedUpdate;
        }

        public State
        (
            T id, 
            string name, 
            DelegateNoArg onEnter, 
            DelegateNoArg onExit = null, 
            DelegateNoArg onUpdate = null, 
            DelegateNoArg onFixedUpdate = null
        ) : this(id, name)
        {
            OnEnter = onEnter;
            OnExit = onExit;
            OnUpdate = onUpdate;
            OnFixedUpdate = onFixedUpdate;
        }

        public virtual void Enter()
        {
            OnEnter?.Invoke();
        }

        public virtual void Exit()
        {
            OnExit?.Invoke();
        }

        public virtual void Update()
        {
            OnUpdate?.Invoke();
        }

        public virtual void FixedUpdate()
        {
            OnFixedUpdate?.Invoke();
        }
    }
}