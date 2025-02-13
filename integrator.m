classdef integrator < handle
  properties (Access = protected)
    method; % Integration method: 'left', 'right', 'middle', 'trapezes', 'gauss2', 'gauss3'
    xk; % interpolation points in [0,1]
    wk; % weights for approximation
    dx; % step size
  endproperties

  methods (Access = public)
    % Constructor initializes method and dx
    function obj = integrator(varargin)
    
      obj.method = "trapezes"; obj.dx = 0.1;
      obj.xk = []; obj.wk = [];

      if ~isempty(varargin)
        for i = 1:2:length(varargin)
          obj.set(varargin{i}, varargin{i+1}); % Set properties if provided
        endfor
      endif

      obj.set_gauss_weights(); % set Gauss weights
    endfunction

    % Get property value
    function retval = get(this, prop)

      if strcmp(prop, "method")
        retval = this.method;
      elseif strcmp(prop, "dx")
        retval = this.dx;
      else
        retval = [];
      endif
    endfunction

    % Set properties
    function this = set(this, varargin)

      for i = 1:2:length(varargin)
        prop = varargin{i}; val = varargin{i+1};
        if strcmp(prop, "method")
          this.method = val;
          this.set_gauss_weights();
        elseif strcmp(prop, "dx")
          this.dx = val;
        endif
      endfor
    endfunction

    % Gauss quadrature weights
    function set_gauss_weights(this)
      if strcmp(this.method, "gauss2")
        this.xk = [0.2113, 0.7887]; this.wk = [0.5, 0.5];
      elseif strcmp(this.method, "gauss3")
        this.xk = [0.1127, 0.5, 0.8873]; this.wk = [5/18, 8/18, 5/18];
      else
        this.xk = []; this.wk = [];
      endif
    endfunction

    % Display method and dx
    function disp(this)
      disp(["method: ", this.method]);
      disp(["dx = ", num2str(this.dx)]);
    endfunction

    % Integrate f over [a, b] with n subintervals
    function I = integrate(this, f, a, b, n, hax)
      h = (b - a) / n; I = 0;

      switch this.method
        case "left"
          for i = 0:n-1, I += f(a + i*h); endfor
          I = I * h;

        case "right"
          for i = 1:n, I += f(a + i*h); endfor
          I *= h;

        case "middle"
          for i = 0:n-1
            I += f(a + (i + 0.5)*h);
          endfor
          I = I * h;

        case "trapezes"
          I = (f(a) + f(b)) / 2;
          for i = 1:n-1, I += f(a + i*h); endfor
          I = h * I;

        case {"gauss2", "gauss3"}
          for i = 0:n-1
            ai = a + i*h; bi = ai + h;
            sum_gauss = 0;
            for j = 1:length(this.xk)
              sum_gauss += this.wk(j) * f(ai + this.xk(j)*(bi - ai)); % Source maths TD
            endfor
            I += (bi - ai) * sum_gauss;
          endfor

        otherwise
          error(["Unknown method: ", this.method]);
      endswitch
    endfunction

    % Compute primitive of f from 'from' to x
    function y = primitive(this, f, from, x)

      y = zeros(size(x));
      for i = 1:length(x)
        n = max(1, round((x(i) - from) / this.dx));
        y(i) = this.integrate(f, from, x(i), n, []);
      endfor
    endfunction

    % Error analysis: |Error| = C / n^alpha
    function retval = integration_error(this, f, a, b, Ith, ns, hax)

      errors = zeros(size(ns));
      for i = 1:length(ns)
        approx = this.integrate(f, a, b, ns(i), []);
        errors(i) = abs(approx - Ith);
      endfor

      p = polyfit(log(ns), log(errors), 1);
      alpha = -p(1); C = exp(p(2));
      retval = [C, alpha];

      if nargin == 6 && ishandle(hax)
        semilogx(ns, errors, "o-");
        xlabel("n"); ylabel("Error");
        title("Error Study"); grid on;
      endif
    endfunction
  endmethods
endclassdef